//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2356/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2356<F: Float>(t22480: F, t4028: F, t12545: F, t12734: F, t1774: F, t22461: F, t22600: F, t2314: F, t2364: F, t24999: F, t25965: F, t4077: F, t6517: F, t7472: F, t91578: F, t91580: F, t91582: F, t91585: F, t91587: F, t91589: F, t91591: F, t91593: F, t91602: F, t91606: F, t91608: F, t91610: F) -> F {
    let t91612 = F::new(2.0) * t4028 * t22480;
    let t91617 = -F::new(4.0) * t12545 * t6517 - F::new(4.0) * t12734 * t7472 - F::new(2.0) * t1774 * t22600 - F::new(4.0) * t22461 * t4077 - F::new(4.0) * t2314 * t25965 - F::new(2.0) * t2364 * t24999 - t91578 - t91580 + t91582 + t91585 - t91587 - t91589 - t91591 - t91593 - t91602 - t91606 - t91608 - t91610 - t91612;
    t91617
}
