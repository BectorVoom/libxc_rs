//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1011/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1011<F: Float>(t21114: F, t932: F, t1557: F, t17195: F, t4354: F, t5727: F, t13520: F, t5730: F, t21252: F, t2844: F, t10661: F, t10675: F, t10676: F, t21120: F, t21124: F, t21128: F, t21132: F, t21136: F, t21140: F, t21142: F, t21144: F, t21147: F, t21150: F, t21153: F, t21156: F) -> (F, F, F, F, F, F, F) {
    let t21259 = t21114 * t932;
    let t21263 = F::new(3.0) * t17195 * t1557;
    let t21265 = F::new(3.0) * t4354 * t5727;
    let t21267 = F::cast_from(0.48245938496077605201e2_f64) * t13520 * t5730;
    let t21268 = t21252 * t2844;
    let t21270 = F::cast_from(0.96491876992155210402e2_f64) * t10661 * t21268;
    let t21283 = F::cast_from(0.16431333333333333333e0_f64) * t21120 - F::cast_from(0.59793333333333333333e0_f64) * t21124 + F::new(0.17938e1) * t21128 - F::cast_from(0.36514074074074074075e-1_f64) * t21132 - F::cast_from(0.82156666666666666667e-1_f64) * t21136 - F::cast_from(0.49293999999999999999e0_f64) * t21140 - F::new(0.28483875e1) * t21142 + F::new(0.46074375e0) * t21144 - t10675 - t10676 - F::cast_from(0.33218518518518518518e0_f64) * t21147 + F::cast_from(0.11958666666666666667e1_f64) * t21150 - F::new(0.17938e1) * t21153 - F::cast_from(0.29896666666666666667e0_f64) * t21156;
    (t21259, t21263, t21265, t21267, t21268, t21270, t21283)
}
