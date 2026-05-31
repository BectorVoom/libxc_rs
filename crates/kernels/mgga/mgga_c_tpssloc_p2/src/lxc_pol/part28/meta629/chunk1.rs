//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1971/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1971<F: Float>(t86886: F, t86895: F, t2053: F, t40889: F, t10049: F, t13049: F, t25168: F, t26713: F, t2743: F, t7842: F, t866: F, t86847: F, t86852: F, t86857: F, t86862: F, t86866: F, t86875: F, t86881: F, t86884: F, t86891: F, t86901: F, t86903: F, t92375: F, t92382: F, t92383: F, t92386: F) -> F {
    let t92390 = F::cast_from(0.15352717957250113407e0_f64) * t86886;
    let t92393 = F::cast_from(0.3289868133696452873e-1_f64) * t86895;
    let t92394 = t40889 * t2053;
    let t92400 = t92375 + F::cast_from(0.3289868133696452873e-1_f64) * t86847 - t26713 * t2743 + F::cast_from(0.6579736267392905746e-1_f64) * t86852 + F::cast_from(0.6579736267392905746e-1_f64) * t86857 + F::cast_from(0.6579736267392905746e-1_f64) * t86862 + F::cast_from(0.3289868133696452873e-1_f64) * t86866 + t92382 - t92383 + F::cast_from(0.6579736267392905746e-1_f64) * t86875 - F::cast_from(0.9869604401089358619e-1_f64) * t86881 - F::cast_from(2.0_f64) * t92386 * t866 + F::cast_from(0.6579736267392905746e-1_f64) * t86884 + t92390 - t10049 * t7842 - F::cast_from(0.3289868133696452873e-1_f64) * t86891 + t92393 + F::cast_from(24.0_f64) * t25168 * t92394 * t13049 + F::cast_from(0.9869604401089358619e-1_f64) * t86901 - F::cast_from(0.25587863262083522346e0_f64) * t86903;
    t92400
}
