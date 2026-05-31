//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2283/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2283<F: Float>(t18029: F, t6754: F, t1025: F, t1618: F, t1622: F, t17693: F, t17697: F, t17734: F, t23537: F, t23544: F, t25577: F, t25580: F, t4636: F, t4652: F, t5900: F, t6765: F, t82914: F, t88277: F, t88305: F, t88307: F, t88388: F) -> F {
    let t99539 = t18029 * t6754;
    let t99556 = t88305 - t88307 - t82914 / F::cast_from(6912.0_f64) - t23544 * t5900 / F::cast_from(1152.0_f64) + t99539 * t1025 / F::cast_from(1536.0_f64) + t88388 * t1618 / F::cast_from(768.0_f64) + t25577 * t4652 / F::cast_from(768.0_f64) + t88277 * t1622 / F::cast_from(1152.0_f64) + t25580 * t4636 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t6765 * t17693 + F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t6765 * t17697 + t23537 * t17734 / F::cast_from(384.0_f64);
    t99556
}
