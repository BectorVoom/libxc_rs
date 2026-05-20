//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1835;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1836;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta379<F: Float>(t13851: F, t2990: F, t10245: F, t4531: F, t10241: F, t4514: F, t2989: F, t3966: F, t2988: F, t13542: F, t4518: F, t13546: F, t10259: F, t13559: F, t13555: F, t4510: F, t1597: F, t3014: F, t343: F, t4546: F, t3008: F, t2960: F, t4506: F, t10263: F, t13850: F, t1593: F, t2986: F, t973: F) -> (F, F, F, F, F) {
        let (t13852, t13855, t13858, t13861) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1835::<F>(t13851, t2990, t10245, t4531, t10241, t4514, t2989, t3966);
        let (t13862, t13865, t13868, t13871, t13874, t13877, t13881) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1836::<F>(t13861, t2988, t13542, t4518, t13546, t10259, t4514, t13559, t13555, t4510, t1597, t3014, t343);
        let (t13886, t13893, t13894) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1837::<F>(t13881, t4546, t1597, t3008, t343, t2960, t4506, t10263, t13850, t13852, t13855, t13858, t13862, t13865, t13868, t13871, t13874, t13877, t1593, t2986, t973);
    (t13861, t13881, t13886, t13893, t13894)
}
