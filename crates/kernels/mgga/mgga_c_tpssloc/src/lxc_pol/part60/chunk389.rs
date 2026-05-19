//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 389/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk389<F: Float>(t2859: F, t302: F, t2764: F, t2822: F, t922: F, t310: F, t320: F, t941: F, t315: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2860 = F::new(1.0) / t2859;
    let t2861 = t302 * t2860;
    let t2868 = F::cast_from(0.68863333333333333333e0_f64) * t2764;
    let t2875 = F::cast_from(0.17365833333333333333e0_f64) * t2822;
    let t2884 = t922 * t922;
    let t2885 = F::new(1.0) / t2884;
    let t2886 = t302 * t2885;
    let t2887 = t310 * t310;
    let t2888 = F::new(1.0) / t2887;
    let t2892 = F::cast_from(0.12361111111111111111e-1_f64) * t2764;
    let t2903 = t941 * t320;
    let t2904 = F::new(1.0) / t2903;
    let t2905 = t315 * t2904;
    let t2912 = F::cast_from(0.40256666666666666667e0_f64) * t2764;
    let t2919 = F::new(0.137975e0) * t2822;
    let t2928 = t941 * t941;
    let t2929 = F::new(1.0) / t2928;
    let t2930 = t315 * t2929;
    (t2861, t2868, t2875, t2886, t2888, t2892, t2904, t2905, t2912, t2919, t2929, t2930)
}
