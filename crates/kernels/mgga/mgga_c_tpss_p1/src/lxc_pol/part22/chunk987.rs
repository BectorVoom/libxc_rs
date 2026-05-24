//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 987/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk987<F: Float>(t2383: F, t3689: F, t10575: F, t10581: F, t10587: F, t10592: F, t10596: F, t10600: F, t10602: F, t10606: F, t10610: F, t10614: F, t10617: F, t2173: F, t3626: F) -> F {
    let t10620 = F::new(7.0) / F::new(576.0) * t2383 * t3689;
    let t10621 = -F::new(5.0) / F::new(384.0) * t2173 * t10575 + t2173 * t10581 / F::new(384.0) - t3626 * t10587 / F::new(192.0) - t2173 * t10592 / F::new(1536.0) - t2173 * t10596 / F::new(3072.0) + t10600 + t2173 * t10602 / F::new(384.0) + t2173 * t10606 / F::new(768.0) + t3626 * t10610 / F::new(768.0) + t3626 * t10614 / F::new(1536.0) - F::new(119.0) / F::new(3456.0) * t10617 + t10620;
    t10621
}
