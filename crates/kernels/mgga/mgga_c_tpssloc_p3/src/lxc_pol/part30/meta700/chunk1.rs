//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2254/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2254<F: Float>(t81912: F, t87412: F, t87426: F, t92676: F, t92677: F, t92689: F, t98818: F, t98820: F, t98822: F, t98824: F, t98826: F, t98828: F, t98830: F, t98833: F, t98836: F, t98838: F, t98842: F, t98844: F) -> F {
    let t98846 = t92676 - t92677 + t87412 - t98818 / F::cast_from(384.0_f64) - t98820 / F::cast_from(384.0_f64) - t98822 / F::cast_from(192.0_f64) - t98824 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t98826 - F::cast_from(35.0_f64) / F::cast_from(576.0_f64) * t98828 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t98830 - t98833 / F::cast_from(384.0_f64) - F::cast_from(0.11304371706359309439e-1_f64) * t81912 - F::cast_from(0.28260929265898273598e-2_f64) * t98836 - t87426 - t92689 - F::cast_from(0.16956557559538964158e-1_f64) * t98838 - F::cast_from(0.12111826828242117256e-2_f64) * t98842 + t98844 / F::cast_from(192.0_f64);
    t98846
}
