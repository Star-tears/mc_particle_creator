use rand::Rng;

use crate::pre_load::graph::Point;

use super::ve::Ve;

/**
 * 与y坐标无关，将视为xOz平面
 */
pub fn gen_rand_circle_center_point(p1: Point, p2: Point) -> Point {
    let mut rng = rand::thread_rng();
    let k = rng.gen_range(-5.0..=5.0);
    let v1 = Ve::from(p1);
    let v2 = Ve::from(p2);
    let v_mid = Ve {
        x: (v1.x + v2.x) / 2.0,
        y: (v1.y + v2.y) / 2.0,
        z: (v1.z + v2.z) / 2.0,
    };
    let v1_to_v2 = v2.sub(&v1);
    let mut v3 = Ve {
        x: v1_to_v2.z * (-1.0),
        y: 0.0,
        z: v1_to_v2.x,
    };
    v3.normalized();
    v_mid.add(&v3.multi_f64(k)).to_point()
}

pub fn get_dest_cir_center(pa: Point, pb: Point, pc: Point) -> Option<Point> {
    let va = Ve::from(pa);
    let vb = Ve::from(pb);
    let vc = Ve::from(pc);
    let mut vc_to_va = va.sub(&vc);
    let va_to_vb = vb.sub(&va);
    if eq_f64(vc_to_va.dot_product(&va_to_vb), 0.0) {
        return None;
    }
    let cos_alpha = vc_to_va.get_cos_alpha(&va_to_vb);
    vc_to_va.normalized();
    let d = pa.dist_to(&pb) / 2.0;
    Some(va.add(&vc_to_va.multi_f64(d / cos_alpha)).to_point())
}

pub fn eq_f64(a: f64, b: f64) -> bool {
    (a - b).abs().le(&1e-8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_a_circle_center() {
        let p1 = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let p2 = Point {
            x: 4.0,
            y: 0.0,
            z: 5.0,
        };
        let center_p1 = gen_rand_circle_center_point(p1, p2);
        let center_p2 = gen_rand_circle_center_point(p1, p2);
        println!("{:#?}", center_p1);
        println!("{:#?}", center_p2);
        assert!(!center_p1.eq(&center_p2));
        assert!((center_p1.dist_to(&p1) - center_p1.dist_to(&p2))
            .abs()
            .le(&1e-10));
        assert!((center_p2.dist_to(&p1) - center_p2.dist_to(&p2))
            .abs()
            .le(&1e-10));
    }
}
