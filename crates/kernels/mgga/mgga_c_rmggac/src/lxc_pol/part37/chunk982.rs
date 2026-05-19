//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 982/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk982<F: Float>(t7717: F, t77768: F, t75675: F, t75681: F, t75685: F, t75687: F, t75705: F, t1356: F, t37423: F, t8936: F, t14451: F, t5267: F) -> (F, F, F, F, F, F, F, F) {
    let t77769 = t7717 * t77768;
    let t77770 = F::cast_from(0.53205749866622299248e-5_f64) * t77769;
    let t77772 = F::cast_from(0.79828278012425390427e-1_f64) * t75675;
    let t77773 = F::cast_from(0.1276937996798935182e-4_f64) * t75681;
    let t77774 = F::cast_from(0.15961724959986689775e-4_f64) * t75685;
    let t77775 = F::cast_from(0.1276937996798935182e-4_f64) * t75687;
    let t77782 = F::cast_from(0.44903406381989282115e-1_f64) * t75705;
    let t77785 = F::cast_from(0.11974241701863808564e0_f64) * t1356 * t37423 * t8936;
    let t77786 = t14451 * t5267;
    (t77770, t77772, t77773, t77774, t77775, t77782, t77785, t77786)
}
