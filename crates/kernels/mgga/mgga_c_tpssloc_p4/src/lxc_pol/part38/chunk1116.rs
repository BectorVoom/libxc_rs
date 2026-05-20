//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1116/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1116<F: Float>(t13881: F, t4546: F, t1597: F, t3008: F, t343: F, t2960: F, t4506: F, t10263: F, t13850: F, t13852: F, t13855: F, t13858: F, t13862: F, t13865: F, t13868: F, t13871: F, t13874: F, t13877: F, t1593: F, t2986: F, t973: F) -> F {
    let t13882 = t4546 * t13881;
    let t13886 = t1597 * t3008 * t343;
    let t13887 = t4546 * t13886;
    let t13893 = F::cast_from(0.49382716049382716048e-3_f64) * t2960 * t4506;
    let t13894 = -t13850 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t13852 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t13855 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t13858 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t13862 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t13865 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t13868 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t13871 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t13874 - F::cast_from(0.22222222222222222221e-2_f64) * t2986 * t13877 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t13882 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t13887 + F::cast_from(0.27160493827160493826e-2_f64) * t10263 * t1593 - t13893;
    t13894
}
