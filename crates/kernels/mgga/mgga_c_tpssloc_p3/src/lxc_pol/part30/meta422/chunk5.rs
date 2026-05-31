//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1626/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1626<F: Float>(t3640: F, t6270: F, t11947: F, t6274: F, t1254: F, t18682: F, t18685: F, t18688: F, t18690: F, t18692: F, t18694: F, t18696: F, t18837: F, t18839: F, t18917: F, t18920: F, t18922: F, t18924: F, t18928: F, t18930: F, t18932: F, t18936: F, t18938: F, t4700: F) -> F {
    let t19267 = t6270 * t3640;
    let t19270 = t6274 * t11947;
    let t19274 = -t1254 * t19267 * t4700 + F::cast_from(2.0_f64) * t1254 * t19270 * t4700 - t18682 - t18685 + t18688 + t18690 + t18692 - t18694 + t18696 + t18837 + t18839 - t18917 + t18920 + t18922 - t18924 - t18928 + t18930 + t18932 + t18936 - t18938;
    t19274
}
