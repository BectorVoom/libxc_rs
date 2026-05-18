//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1098/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1098<F: Float>(t25140: F, t25144: F, t23125: F, t23135: F, t24230: F, t24231: F, t25142: F, t25147: F, t25149: F, t25151: F, t25156: F, t23043: F, t23063: F, t23071: F, t23084: F, t25065: F, t25069: F, t25071: F, t25073: F, t25107: F, t25109: F, t25113: F, t25117: F, t25121: F, t25124: F, t25126: F, t25128: F, t25133: F, t25136: F, t26619: F, t26621: F, t26630: F) -> F {
    let t26644 = F::new(7.0) / F::new(72.0) * t25140;
    let t26646 = F::new(7.0) / F::new(1152.0) * t25144;
    let t26651 = F::new(0.40372756094140390853e-3) * t23125 + t26644 + F::new(5.0) / F::new(192.0) * t25142 + t26646 - t25147 / F::new(768.0) - t25149 / F::new(768.0) - t25151 / F::new(768.0) + t23135 + t24230 + t24231 + t25156 / F::new(8.0);
    let t26653 = F::new(0.40372756094140390853e-3) * t25065 + t23043 - t25069 / F::new(192.0) - t25071 / F::new(192.0) - t25073 / F::new(192.0) + F::new(0.16956557559538964158e-1) * t23063 + t23071 + t26619 + F::new(0.28260929265898273597e-2) * t23084 - t26621 + t26630 - F::new(0.24223653656484234512e-2) * t25107 + F::new(0.16956557559538964158e-1) * t25109 + F::new(0.24223653656484234512e-2) * t25113 - F::new(0.40372756094140390853e-3) * t25117 + F::new(0.16956557559538964158e-1) * t25121 - F::new(0.40372756094140390853e-3) * t25124 + F::new(0.28260929265898273597e-2) * t25126 - t25128 / F::new(24.0) + F::new(0.67287926823567318088e-4) * t25133 + t25136 / F::new(768.0) + t26651;
    t26653
}
