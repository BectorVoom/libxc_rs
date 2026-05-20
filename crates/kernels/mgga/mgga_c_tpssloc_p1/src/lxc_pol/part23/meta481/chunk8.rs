//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1447/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1447<F: Float>(t78423: F, t78441: F, t78460: F, t78489: F, t78516: F, t78545: F, t78578: F, t78634: F, t1238: F, t1751: F, t1760: F, t1761: F, t19232: F, t19234: F, t22004: F, t22008: F, t22113: F, t22393: F, t22394: F, t3598: F, t491: F, t4945: F, t498: F, t5055: F, t6150: F, t6238: F, t6244: F, t6268: F, t73900: F, t78379: F) -> (F, F) {
    let t78637 = t78423 + t78441 + t78460 + t78489 + t78516 + t78545 + t78578 + t78634;
    let t78646 = F::new(8.0) * t1238 * t1760 * t22393 * t3598 + F::new(6.0) * t1238 * t3598 * t78379 + F::new(4.0) * t1751 * t22113 * t498 + t491 * t498 * t78637 + F::new(6.0) * t498 * t6150 * t6238 - F::new(4.0) * t1761 * t73900 - F::new(6.0) * t19232 * t6268 + F::new(24.0) * t19234 * t6244 - F::new(12.0) * t19234 * t6268 + F::new(24.0) * t22004 * t4945 + F::new(24.0) * t22004 * t5055 - F::new(24.0) * t22008 * t4945 - F::new(4.0) * t22394 * t5055;
    (t78637, t78646)
}
