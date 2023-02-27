pub mod cli {
    fn main() -> () {
        ()
    }
}
trait Hungry {
    fn closer<'a>(&self, first: &'a Restaurant, second: &'a Restaurant) -> &'a Restaurant;
}
struct Restaurant {
    name: String,
    lat: f32,
    long: f32,
}
impl Restaurant {
    fn new(name: &str, lat: f32, long: f32) -> Self {
        Restaurant {
            name: name.to_string(),
            lat,
            long,
        }
    }
    fn distance(&self, rest: &Restaurant) -> f32 {
        ((self.long - rest.long).powi(2) + (self.lat - rest.lat).powi(2)).powf(0.5)
    }
    fn closer<'a>(&self, first: &'a Restaurant, second: &'a Restaurant) -> &'a Restaurant {
        if self.distance(first) >= self.distance(second) {
            first
        } else {
            second
        }
    }
}
fn greedy_path<'a>(
    long: f32,
    lat: f32,
    restaurants: &mut Vec<&'a Restaurant>,
) -> Vec<&'a Restaurant> {
    let mut ret: Vec<&Restaurant> = Vec::new();
    while restaurants.len() > 0 {
        let closest: (usize, f32) =
            restaurants
                .iter()
                .enumerate()
                .fold((0, f32::MAX), |mut acc: (usize, f32), (i, r)| {
                    let d = ((long - r.long).powi(2) + (lat - r.lat).powi(2)).powf(0.5);
                    if d < acc.1 as f32 {
                        acc = (i, d)
                    }
                    acc
                });
        let next = restaurants.remove(closest.0);
        ret.push(next);
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init() {
        let first = Restaurant::new("pho 99", 5.0, 5.0);
        let second = Restaurant::new("burrito king", 90.0, 90.0);
        let mut rests = &mut vec![&first, &second];
        let ret = greedy_path(0.0, 0.0, &mut rests);
        assert_eq!(ret[0].name, first.name);
    }
}
