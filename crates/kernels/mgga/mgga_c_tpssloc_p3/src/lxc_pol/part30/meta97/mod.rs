//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk630;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk631;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk632;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk633;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk634;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk635;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk636;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta97<F: Float>(t2006: F, t539: F, t1998: F, t562: F, t214: F, t1985: F, t553: F, t544: F, t1378: F, t1375: F, t1989: F, t568: F, t533: F, t1390: F, t1983: F, t113: F, t1869: F, t1876: F, t1976: F, t1980: F, t510: F, t574: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2007, t2009, t2010, t2011, t2013) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk630::<F>(t2006, t539, t1998, t562, t214, t1985, t553);
        let t2015 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk631::<F>(t2011, t2013, t544);
        let t2016 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk632::<F>(t1378, t2015);
        let t2018 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk633::<F>(t1375, t1989, t2007, t2016, t568);
        let t2019 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk634::<F>(t2018, t533);
        let t2020 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk635::<F>(t1390, t2019);
        let t2022 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk636::<F>(t1983, t2020, t113, t1869, t1876, t1976, t1980, t510, t574);
    (t2007, t2009, t2010, t2013, t2015, t2016, t2018, t2019, t2020, t2022)
}
