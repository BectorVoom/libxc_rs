//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 841/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk841<F: Float>(t10768: F, t10847: F, t300: F, t2940: F, t2944: F, t2924: F, t2929: F, t4497: F, t959: F, t10665: F, t10699: F, t10707: F, t10711: F, t10715: F, t10729: F, t10733: F, t10739: F, t10819: F) -> (F, F, F, F) {
    let t10849 = t300 * (t10768 + t10847);
    let t10851 = F::cast_from(0.35089341735807877242e1_f64) * t2940 * t2944;
    let t10853 = t2929 * t2924 * t4497;
    let t10855 = F::cast_from(0.51947577317044391277e2_f64) * t959 * t10853;
    let t10856 = -t10665 + t10699 + t10707 - t10711 + t10715 + t10849 - t10819 + t10739 - t10729 + t10733 + t10851 - t10855;
    (t10849, t10851, t10855, t10856)
}
