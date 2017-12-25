use super::PhotoLink;
use super::views_by_date::query_date;
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use models::Photo;
use nickel::Request;
use nickel_diesel::DieselRequestExtensions;
use schema::photos;

pub fn links_by_time<'a>(
    req: &mut Request,
    photos: photos::BoxedQuery<'a, Pg>,
) -> Vec<PhotoLink> {
    let c: &PgConnection = &req.db_conn();
    use schema::photos::dsl::date;
    let photos = if let Some(from_date) = query_date(req, "from") {
        photos.filter(date.ge(from_date))
    } else {
        photos
    };
    let photos = if let Some(to_date) = query_date(req, "to") {
        photos.filter(date.le(to_date))
    } else {
        photos
    };
    let photos = photos.order(date.desc().nulls_last()).load(c).unwrap();
    if let Some(groups) = split_to_groups(&photos) {
        let path = req.path_without_query().unwrap_or("/");
        groups
            .iter()
            .map(|g| PhotoLink::for_group(g, path))
            .collect::<Vec<_>>()
    } else {
        photos.iter().map(PhotoLink::from).collect::<Vec<_>>()
    }
}

pub fn split_to_groups(photos: &[Photo]) -> Option<Vec<&[Photo]>> {
    let wanted_groups = match photos.len() {
        l if l <= 16 => return None,
        l if l < 81 => 8,
        l if l >= 225 => 15,
        l => (l as f64).sqrt() as usize,
    };
    let mut groups = vec![photos];
    while groups.len() < wanted_groups {
        let i = find_largest(&groups);
        let (a, b) = split(groups[i]);
        groups[i] = a;
        groups.insert(i + 1, b);
    }
    Some(groups)
}

fn find_largest(groups: &[&[Photo]]) -> usize {
    let mut found = 0;
    let mut largest = 0.0;
    for (i, g) in groups.iter().enumerate() {
        let time = 1 + g.first().map(|p| timestamp(p)).unwrap_or(0)
            - g.last().map(|p| timestamp(p)).unwrap_or(0);
        let score = (g.len() as f64).powi(3) * (time as f64);
        if score > largest {
            largest = score;
            found = i;
        }
    }
    found
}

fn split(group: &[Photo]) -> (&[Photo], &[Photo]) {
    let l = group.len();
    let edge = l / 16;
    let mut pos = 0;
    let mut largest = 0;
    for i in edge..l - 1 - edge {
        let interval = timestamp(&group[i]) - timestamp(&group[i + 1]);
        if interval > largest {
            largest = interval;
            pos = i + 1;
        }
    }
    group.split_at(pos)
}

fn timestamp(p: &Photo) -> i64 {
    p.date.map(|d| d.timestamp()).unwrap_or(0)
}
