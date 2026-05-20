//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk601;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk602;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk603;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk604;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk605;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta92<F: Float>(t209: F, t540: F, t1878: F, t225: F, t562: F, t567: F, t214: F, t1887: F, t534: F, t532: F, t556: F, t598: F, t213: F, t552: F, t236: F, t553: F, t59: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1984, t1985) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk601::<F>(t209, t540, t1878);
        let (t1987, t1988, t1989, t1992) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk602::<F>(t225, t562, t567, t214, t1985, t1878, t1887, t534);
        let t1995 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk603::<F>(t532, t556);
        let (t1996, t1997, t1998) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk604::<F>(t1995, t598, t213, t225, t552);
        let t1999 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk605::<F>(t1998, t236);
        let (t2000, t2002) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk606::<F>(t1997, t1999, t553, t59);
    (t1984, t1985, t1987, t1988, t1989, t1992, t1995, t1996, t1998, t1999, t2000, t2002)
}
