//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 960/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk960<F: Float>(t23125: F, t23135: F, t24230: F, t24231: F, t25142: F, t25147: F, t25149: F, t25151: F, t25156: F, t26644: F, t26646: F, t23043: F, t23063: F, t23071: F, t23084: F, t25065: F, t25069: F, t25071: F, t25073: F, t25107: F, t25109: F, t25113: F, t25117: F, t25121: F, t25124: F, t25126: F, t25128: F, t25133: F, t25136: F, t26619: F, t26621: F, t26630: F) -> (F,) {
    let t26651 = 0.40372756094140390853e-3 * t23125 + t26644 + 5.0 / 192.0 * t25142 + t26646 - t25147 / 768.0 - t25149 / 768.0 - t25151 / 768.0 + t23135 + t24230 + t24231 + t25156 / 8.0;
    let t26653 = 0.40372756094140390853e-3 * t25065 + t23043 - t25069 / 192.0 - t25071 / 192.0 - t25073 / 192.0 + 0.16956557559538964158e-1 * t23063 + t23071 + t26619 + 0.28260929265898273597e-2 * t23084 - t26621 + t26630 - 0.24223653656484234512e-2 * t25107 + 0.16956557559538964158e-1 * t25109 + 0.24223653656484234512e-2 * t25113 - 0.40372756094140390853e-3 * t25117 + 0.16956557559538964158e-1 * t25121 - 0.40372756094140390853e-3 * t25124 + 0.28260929265898273597e-2 * t25126 - t25128 / 24.0 + 0.67287926823567318088e-4 * t25133 + t25136 / 768.0 + t26651;
    (t26653,)
}
