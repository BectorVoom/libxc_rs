//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1287/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1287<F: Float>(t31776: F, t96797: F, t1983: F, t33136: F, t7217: F, t33623: F, t6876: F, t33214: F, t7057: F, t25985: F, t8607: F, t120071: F, t122088: F, t122094: F, t122583: F, t2096: F, t22461: F, t24980: F, t26103: F, t26969: F, t27226: F, t6517: F, t7042: F, t7802: F, t8450: F) -> (F,) {
    let t122587 = 2.0 * t96797 * t31776;
    let t122589 = t1983 * t7217 * t33136;
    let t122590 = t6876 * t33623;
    let t122593 = 2.0 * t33214 * t7057;
    let t122595 = 3.0 * t8607 * t25985;
    let t122596 = t120071 * t2096 - 2.0 * t22461 * t7802 - 2.0 * t24980 * t7042 - 2.0 * t26103 * t7802 + 3.0 * t26969 * t8450 - 2.0 * t27226 * t6517 + t122088 + t122094 + t122583 + t122587 - t122589 - t122590 - t122593 + t122595;
    (t122596,)
}
