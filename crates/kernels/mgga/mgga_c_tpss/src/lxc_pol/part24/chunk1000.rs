//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1000/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1000<F: Float>(t4683: F, t8082: F, t3572: F, t3642: F, t2440: F, t4706: F, t3553: F, t3610: F, t3569: F, t4744: F, t72: F, t732: F, t14003: F, t14029: F, t198: F, t2439: F, t3548: F, t3552: F, t740: F, t7929: F, t7932: F, t7936: F, t8000: F, t8019: F, t8023: F, t8024: F, t8029: F, t8030: F, t8040: F) -> (F, F, F, F, F) {
    let t14034 = 12.0 * t8082 * t4683;
    let t14035 = t3572 * t3642;
    let t14036 = 8.0 * t14035;
    let t14040 = t2440 * t4706;
    let t14046 = t3553 * t3610;
    let t14050 = 8.0 * t3572 * t3569;
    let t14051 = t4744 * t72;
    let t14052 = t14051 * t732;
    let t14053 = 0.18311447306006545054e-3 * t14052;
    let t14054 = 3.0 * t14029 * t198 * t740 + 6.0 * t198 * t4706 * t8030 + 6.0 * t2439 * t3548 * t3610 + 6.0 * t14040 * t3552 + 12.0 * t14046 * t3552 - t14003 + t14034 + t14036 + t14050 - t14053 + t7929 - t7932 - t7936 + t8000 - t8019 + t8023 + t8024 - t8029 - t8040;
    (t14034, t14036, t14050, t14053, t14054)
}
