//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2344/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2344<F: Float>(t20908: F, t2697: F, t1509: F, t5611: F, t13222: F, t13251: F, t16914: F, t16924: F, t17009: F, t20896: F, t2623: F, t2643: F, t2647: F, t46692: F, t47044: F, t47047: F, t5593: F, t58859: F, t58873: F, t58885: F, t58890: F, t58900: F, t829: F) -> F {
    let t68021 = t2697 * t20908;
    let t68025 = t5611 * t1509;
    let t68048 = F::new(7.0) / F::new(1152.0) * t68021 - F::new(5.0) / F::new(128.0) * t2623 * t20896 - t2643 * t46692 * t68025 * t829 / F::new(1024.0) + t2643 * t13222 * t68025 * t2647 / F::new(256.0) + F::new(7.0) / F::new(1536.0) * t58859 - F::new(7.0) / F::new(192.0) * t58873 - t13251 * t17009 / F::new(512.0) - F::new(7.0) / F::new(384.0) * t58885 + F::new(7.0) / F::new(1536.0) * t58890 - F::new(7.0) / F::new(256.0) * t58900 - F::new(595.0) / F::new(3456.0) * t47047 + t47044 * t5593 / F::new(128.0) + t13251 * t16924 / F::new(128.0) + t13251 * t16914 / F::new(128.0);
    t68048
}
