//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1583/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1583<F: Float>(t136: F, t18517: F, t1113: F, t18241: F, t11211: F, t11487: F, t14766: F, t15347: F, t15348: F, t15349: F, t18494: F, t18497: F, t18500: F, t18503: F, t18505: F, t18508: F, t18510: F, t18512: F, t18515: F) -> (F, F, F) {
    let t18518 = t136 * t18517;
    let t18520 = t1113 * t18241;
    let t18521 = t136 * t18520;
    let t18523 = t11487 - F::new(5.0) / F::new(27.0) * t11211 - F::new(10.0) / F::new(27.0) * t14766 - t15347 + t15348 + t15349 - t18494 / F::new(27.0) - F::new(2.0) / F::new(27.0) * t18497 + t18500 / F::new(3.0) + t18503 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t18505 - t18508 - F::new(2.0) / F::new(3.0) * t18510 + t18512 / F::new(9.0) + t18515 / F::new(18.0) - t18518 / F::new(3.0) - t18521 / F::new(6.0);
    (t18518, t18521, t18523)
}
