//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1100/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1100<F: Float>(t22858: F, t1358: F, t6940: F, t1887: F, t22715: F, t534: F, t1995: F, t9223: F, t213: F, t1999: F, t22805: F, t22809: F, t22820: F, t22826: F, t22830: F, t22834: F, t22837: F, t22840: F, t22848: F, t22850: F, t22856: F) -> (F, F, F) {
    let t22859 = F::new(119.0) / F::new(6912.0) * t22858;
    let t22860 = t6940 * t1358;
    let t22861 = F::new(7.0) / F::new(1152.0) * t22860;
    let t22863 = t22715 * t534 * t1887;
    let t22864 = F::new(35.0) / F::new(432.0) * t22863;
    let t22865 = t9223 * t1995;
    let t22866 = t22865 * t213;
    let t22867 = t22866 * t1999;
    let t22868 = F::cast_from(0.11304371706359309439e-1_f64) * t22867;
    let t22869 = F::cast_from(0.16956557559538964159e-1_f64) * t22805 - F::cast_from(0.12111826828242117256e-2_f64) * t22809 - t22820 + t22826 + F::cast_from(0.24223653656484234512e-2_f64) * t22830 + t22834 / F::new(192.0) + t22837 / F::new(1536.0) + t22840 / F::new(16.0) + F::cast_from(0.84782787797694820792e-2_f64) * t22848 + F::new(5.0) / F::new(384.0) * t22850 + F::cast_from(0.6728792682356731809e-4_f64) * t22856 + t22859 - t22861 + t22864 + t22868;
    (t22863, t22865, t22869)
}
