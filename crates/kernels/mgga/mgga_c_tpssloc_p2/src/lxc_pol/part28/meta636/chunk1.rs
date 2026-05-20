//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2021/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2021<F: Float>(t91225: F, t80780: F, t80784: F, t80789: F, t80792: F, t80794: F, t80796: F, t80801: F, t80807: F, t80814: F, t80821: F, t80828: F, t84514: F, t91229: F, t91233: F, t91237: F, t91241: F, t91256: F) -> F {
    let t93682 = F::cast_from(0.56521858531796547194e-2_f64) * t91225;
    let t93699 = t93682 - F::cast_from(0.40372756094140390853e-3_f64) * t91229 - F::cast_from(0.48447307312968469024e-2_f64) * t91233 - F::cast_from(0.24223653656484234512e-2_f64) * t91237 + F::cast_from(0.24223653656484234512e-2_f64) * t91241 - F::cast_from(0.12650130242830655801e-1_f64) * t80780 + F::cast_from(0.67287926823567318088e-4_f64) * t80784 + F::cast_from(0.67287926823567318088e-4_f64) * t80789 - F::cast_from(0.21083550404717759668e-2_f64) * t80792 + F::new(119.0) / F::new(1728.0) * t80794 - F::new(7.0) / F::new(1152.0) * t80796 - F::cast_from(0.13457585364713463618e-3_f64) * t80801 + F::cast_from(0.67287926823567318088e-4_f64) * t80807 + F::cast_from(0.40372756094140390853e-3_f64) * t80814 - F::new(7.0) / F::new(144.0) * t80821 - t84514 - F::new(7.0) / F::new(24.0) * t80828 - F::cast_from(0.16956557559538964158e-1_f64) * t91256;
    t93699
}
