//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 996/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk996<F: Float>(t1880: F, t8547: F, t98133: F, t28263: F, t31366: F, t22986: F, t23270: F, t31337: F, t5544: F, t126413: F, t31332: F, t1888: F, t33457: F, t86873: F) -> (F, F, F, F, F) {
    let t127798 = t1880 * t98133 * t8547;
    let t127803 = t1880 * t31366 * t28263;
    let t127814 = t22986 * t23270 * t31337 * t5544;
    let t127818 = t22986 * t23270 * t31332 * t126413;
    let t127829 = t1888 * t86873 * t33457;
    (t127798, t127803, t127814, t127818, t127829)
}
