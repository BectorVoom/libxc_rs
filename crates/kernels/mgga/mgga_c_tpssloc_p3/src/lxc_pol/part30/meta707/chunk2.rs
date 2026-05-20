//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2334/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2334<F: Float>(t3941: F, t4072: F, t7467: F, t28017: F, t3938: F, t12524: F, t28899: F, t100867: F, t100871: F, t100873: F, t100875: F, t100879: F, t100883: F, t100885: F, t100887: F, t100890: F, t20176: F, t23877: F, t23880: F, t26523: F, t5456: F, t5493: F, t577: F, t83980: F, t96351: F) -> F {
    let t100893 = F::new(54.0) * t3941 * t7467 * t4072;
    let t100897 = F::new(0.135e2) * t3938 * t28017;
    let t100899 = F::new(27.0) * t12524 * t28899;
    let t100900 = F::new(27.0) * t26523 * t4072 + F::new(27.0) * t96351 * t5456 + F::new(0.45e1) * t100867 * t577 + t100871 + t100873 + t100875 + F::new(54.0) * t23880 * t20176 + t100879 + F::new(27.0) * t83980 * t5456 + t100883 + t100885 + t100887 + t100890 + t100893 + F::new(0.135e2) * t23877 * t5493 + t100897 + t100899;
    t100900
}
