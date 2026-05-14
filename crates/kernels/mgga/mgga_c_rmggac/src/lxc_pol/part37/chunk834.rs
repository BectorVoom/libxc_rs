//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 834/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk834<F: Float>(t77769: F, t75675: F, t75681: F, t75685: F, t75687: F, t75705: F, t1356: F, t37423: F, t8936: F, t14451: F, t5267: F, t26291: F, t5888: F, t40724: F, t75719: F, t75721: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77770 = 0.53205749866622299248e-5 * t77769;
    let t77772 = 0.79828278012425390427e-1 * t75675;
    let t77773 = 0.1276937996798935182e-4 * t75681;
    let t77774 = 0.15961724959986689775e-4 * t75685;
    let t77775 = 0.1276937996798935182e-4 * t75687;
    let t77782 = 0.44903406381989282115e-1 * t75705;
    let t77785 = 0.11974241701863808564e0 * t1356 * t37423 * t8936;
    let t77786 = t14451 * t5267;
    let t77787 = t26291 * t77786;
    let t77788 = 0.8980681276397856423e-1 * t77787;
    let t77789 = t14451 * t5888;
    let t77790 = t40724 * t77789;
    let t77791 = 0.8980681276397856423e-1 * t77790;
    let t77792 = 0.20455996240684006298e-1 * t75719;
    let t77793 = 0.2727466165424534173e-1 * t75721;
    (t77770, t77772, t77773, t77774, t77775, t77782, t77785, t77786, t77788, t77789, t77791, t77792, t77793)
}
