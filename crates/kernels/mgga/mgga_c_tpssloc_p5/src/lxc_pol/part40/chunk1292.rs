//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1292/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1292<F: Float>(t110082: F, t110097: F, t110102: F, t110103: F, t110503: F, t110506: F, t110510: F, t110520: F, t110521: F, t110601: F, t111326: F, t111331: F, t1444: F, t19492: F, t29903: F, t29922: F, t29926: F, t30164: F, t30175: F, t30410: F, t4067: F, t5396: F, t5464: F, t5468: F, t5488: F, t659: F, t666: F, t8128: F, t8129: F, t8137: F, t8138: F, t96715: F, t96718: F, t96723: F) -> F {
    let t111379 = F::new(44.0) / F::new(9.0) * t110503 + t110506 - F::new(110.0) / F::new(27.0) * t110510 - F::new(20.0) / F::new(9.0) * t111326 - F::new(25.0) / F::new(18.0) * t8128 * t29922 * t30410 + F::new(5.0) / F::new(6.0) * t110601 * t8138 * t111331 - F::new(5.0) / F::new(18.0) * t30175 * t29926 * t19492 + t110102 + F::new(55.0) / F::new(27.0) * t110103 + F::new(3.0) * t110082 * t8129 * t96715 - F::new(5.0) / F::new(4.0) * t29903 * t8138 * t5464 * t659 - F::new(3.0) / F::new(2.0) * t29903 * t8129 * t96718 + F::new(5.0) / F::new(6.0) * t8128 * t8138 * t4067 * t1444 - F::new(3.0) / F::new(4.0) * t29903 * t8129 * t96723 + F::new(5.0) / F::new(12.0) * t8128 * t8138 * t5488 * t659 + F::new(5.0) / F::new(18.0) * t8128 * t29926 * t5468 * t666 + F::new(5.0) / F::new(108.0) * t8137 * t110097 * t5468 * t659 + F::new(5.0) / F::new(12.0) * t8128 * t8138 * t5396 * t666 - F::new(5.0) / F::new(36.0) * t8137 * t29926 * t5396 * t659 - F::new(5.0) / F::new(2.0) * t110520 * t110521 * t30164;
    t111379
}
