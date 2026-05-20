//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2913/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2913<F: Float>(t10760: F, t10828: F, t14266: F, t14329: F, t1569: F, t17350: F, t17428: F, t17499: F, t2856: F, t2881: F, t2889: F, t2906: F, t2907: F, t2924: F, t2930: F, t2932: F, t41816: F, t41826: F, t41981: F, t4411: F, t4434: F, t48771: F, t48779: F, t48890: F, t5743: F, t5759: F, t5794: F, t59975: F, t60407: F, t60424: F, t60429: F, t60434: F, t60568: F, t60570: F, t60585: F, t60601: F, t60618: F, t60634: F, t60649: F, t60665: F, t60682: F, t60698: F, t924: F, t932: F, t950: F) -> F {
    let t60711 = F::cast_from(0.41016075432865626631e4_f64) * t48779 * t48890 * t950 + F::new(1.0) * t17428 * t2881 + F::cast_from(0.32163958997385070134e2_f64) * t60407 * t2889 + F::new(2.0) * t48771 * t1569 + F::new(4.0) * t14266 * t4434 + F::new(2.0) * t4411 * t14329 - F::new(2.0) * t41981 * t5743 + F::new(1.0) * t10760 * t5759 + F::new(2.0) * t2856 * t17350 + F::cast_from(0.17315859105681463759e2_f64) * t41816 * t5794 - F::cast_from(0.11696447245269292414e1_f64) * t60424 * t2907 - t60429 + F::cast_from(0.34631718211362927518e2_f64) * t2930 * t59975 * t2932 - t60434 - t60568 - t60570 + F::new(1.0) * t924 * (t60585 + t60601 + t60618 + t60634 + t60649 + t60665 + t60682 + t60698) * t932 - F::cast_from(0.10389515463408878255e3_f64) * t10828 * t5794 * t2924 - F::cast_from(0.12304822629859687989e5_f64) * t41826 * t17499 * t2906;
    t60711
}
