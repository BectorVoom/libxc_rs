//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2326/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2326<F: Float>(t24682: F, t460: F, t95484: F, t27634: F, t3030: F, t86259: F, t24740: F, t5064: F, t15640: F, t24729: F, t14726: F, t15394: F, t2121: F, t2132: F, t2133: F, t24706: F, t27639: F, t27645: F, t27674: F, t27704: F, t3552: F, t3557: F, t3580: F, t4928: F, t7321: F, t7331: F, t86365: F, t86368: F) -> F {
    let t95678 = t24682 * t95484 * t460;
    let t95682 = t27634 * t86259 * t3030;
    let t95687 = t5064 * t24740;
    let t95702 = t24729 * t15640 / F::new(576.0);
    let t95703 = -F::cast_from(0.20186378047070195428e-3_f64) * t2132 * t2133 * t4928 * t7321 - F::cast_from(0.20186378047070195428e-3_f64) * t95678 * t7331 - F::cast_from(0.40372756094140390856e-3_f64) * t95682 * t27639 + F::cast_from(0.20186378047070195428e-3_f64) * t95682 * t27645 - t95687 * t3580 / F::new(1152.0) - F::cast_from(0.10093189023535097714e-3_f64) * t27704 * t24706 - F::new(7.0) / F::new(648.0) * t2121 * t15394 * t14726 + t86365 / F::new(648.0) - F::cast_from(0.10093189023535097714e-3_f64) * t86368 + t27674 * t3552 / F::new(108.0) + t27674 * t3557 / F::new(54.0) + t95702;
    t95703
}
