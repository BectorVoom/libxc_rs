//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 372/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk372<F: Float>(t2822: F, t241: F, t976: F, t891: F, t275: F, t290: F, t2764: F, t307: F, t922: F, t302: F, t310: F, t320: F, t941: F, t315: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2823 = 0.13692777777777777778e0 * t2822;
    let t2826 = t241 * t976;
    let t2840 = t891 * t891;
    let t2841 = 1.0 / t2840;
    let t2842 = t275 * t2841;
    let t2843 = t290 * t290;
    let t2844 = 1.0 / t2843;
    let t2848 = 0.22831111111111111111e-1 * t2764;
    let t2859 = t922 * t307;
    let t2860 = 1.0 / t2859;
    let t2861 = t302 * t2860;
    let t2868 = 0.68863333333333333333e0 * t2764;
    let t2875 = 0.17365833333333333333e0 * t2822;
    let t2884 = t922 * t922;
    let t2885 = 1.0 / t2884;
    let t2886 = t302 * t2885;
    let t2887 = t310 * t310;
    let t2888 = 1.0 / t2887;
    let t2892 = 0.12361111111111111111e-1 * t2764;
    let t2903 = t941 * t320;
    let t2904 = 1.0 / t2903;
    let t2905 = t315 * t2904;
    let t2912 = 0.40256666666666666667e0 * t2764;
    let t2919 = 0.137975e0 * t2822;
    let t2928 = t941 * t941;
    (t2823, t2826, t2842, t2844, t2848, t2861, t2868, t2875, t2886, t2888, t2892, t2904, t2905, t2912, t2919, t2928)
}
