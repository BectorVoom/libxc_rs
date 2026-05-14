//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 645/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk645<F: Float>(t22867: F, t22805: F, t22809: F, t22820: F, t22826: F, t22830: F, t22834: F, t22837: F, t22840: F, t22848: F, t22850: F, t22856: F, t22859: F, t22861: F, t22864: F, t22802: F) -> (F,) {
    let t22868 = 0.11304371706359309439e-1 * t22867;
    let t22869 = 0.16956557559538964159e-1 * t22805 - 0.12111826828242117256e-2 * t22809 - t22820 + t22826 + 0.24223653656484234512e-2 * t22830 + t22834 / 192.0 + t22837 / 1536.0 + t22840 / 16.0 + 0.84782787797694820792e-2 * t22848 + 5.0 / 384.0 * t22850 + 0.6728792682356731809e-4 * t22856 + t22859 - t22861 + t22864 + t22868;
    let t22870 = t22802 + t22869;
    (t22870,)
}
