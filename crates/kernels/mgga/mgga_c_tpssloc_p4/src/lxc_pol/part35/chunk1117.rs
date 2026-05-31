//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1117/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1117<F: Float>(t22863: F, t1995: F, t9223: F, t213: F, t1999: F, t117: F, t547: F, t67: F, t6559: F) -> (F, F, F, F, F) {
    let t22864 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t22863;
    let t22865 = t9223 * t1995;
    let t22866 = t22865 * t213;
    let t22867 = t22866 * t1999;
    let t22868 = F::cast_from(0.11304371706359309439e-1_f64) * t22867;
    let t22891 = t547 * t67 * t117;
    let t22892 = t6559 * t22891;
    (t22864, t22865, t22868, t22891, t22892)
}
