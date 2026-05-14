//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1066/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1066<F: Float>(t26344: F, t6897: F, t1842: F, t3886: F, t6883: F, t7697: F, t225: F, t7723: F, t22751: F, t7733: F, t22893: F, t7732: F, t22892: F, t1834: F, t552: F, t1824: F, t2006: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26345 = t6897 * t26344;
    let t26354 = t3886 * t1842;
    let t26361 = t6883 * t7697;
    let t26366 = t7723 * t225;
    let t26381 = t22751 * t7733;
    let t26392 = t22893 * t7732;
    let t26393 = t22892 * t26392;
    let t26395 = t552 * t1834;
    let t26403 = t2006 * t1824;
    (t26345, t26354, t26361, t26366, t26381, t26392, t26393, t26395, t26403)
}
