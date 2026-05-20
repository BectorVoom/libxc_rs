//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1970/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1970<F: Float>(t87319: F, t87320: F, t92635: F, t92645: F, t98744: F, t98746: F, t98750: F, t98752: F, t98754: F, t98758: F, t98762: F, t98766: F, t98770: F, t98774: F, t98777: F, t98782: F, t98787: F, t98791: F) -> F {
    let t101456 = F::cast_from(0.33913115119077928316e-1_f64) * t98744 + F::cast_from(0.28260929265898273597e-2_f64) * t98746 - t92635 - F::cast_from(0.80745512188280781707e-3_f64) * t98750 + t98752 / F::new(384.0) - t98754 / F::new(384.0) + F::cast_from(0.24223653656484234512e-2_f64) * t98758 - F::cast_from(0.48447307312968469024e-2_f64) * t98762 + F::cast_from(0.16149102437656156341e-2_f64) * t98766 - F::cast_from(0.16956557559538964158e-1_f64) * t98770 - F::cast_from(0.28260929265898273597e-2_f64) * t98774 + t98777 / F::new(768.0) - F::cast_from(0.13457585364713463618e-3_f64) * t98782 + F::cast_from(0.67287926823567318088e-4_f64) * t98787 + F::cast_from(0.67287926823567318088e-4_f64) * t98791 + t87319 - t87320 - t92645;
    t101456
}
