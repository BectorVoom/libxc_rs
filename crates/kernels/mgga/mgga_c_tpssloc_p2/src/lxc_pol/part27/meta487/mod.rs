//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1868;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1869;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta487<F: Float>(t28: F, t265: F, t504: F, t23772: F, t1972: F, t2250: F, t23820: F, t52: F, t607: F, t6856: F, t23780: F, t1873: F, t3652: F, t652: F, t6876: F, t7000: F, dens_threshold: F, rho1: F, zeta_threshold: F, t6880: F, t9348: F, t12734: F, t2314: F, t6534: F, t12739: F, t5113: F, t1268: F, t22479: F, t22461: F, t22559: F, t22600: F, t2363: F, t6517: F, t671: F, t12461: F, t3698: F, t2019: F, t1983: F, t113: F, t1976: F, t22594: F, t22599: F, t22605: F, t22608: F, t22610: F, t22612: F, t22614: F, t22616: F, t22618: F, t22619: F, t22950: F, t2312: F, t2364: F, t510: F, t574: F) -> (F, F, F, F, F, F, F) {
        let (t23821, t23829, t23831, t23833, t23835) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1868::<F>(t28, t265, t504, t23772, t1972, t2250, t23820, t52, t607, t6856, t23780, t1873, t3652, t652, t6876, t7000, dens_threshold, rho1, zeta_threshold);
        let (t23837, t23855) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1869::<F>(t6876, t6880, t1873, t9348, t12734, t2314, t6534, t12739, t5113, t1268, t22479, t22461, t22559, t22600, t2363, t6517, t671);
        let (t23857, t23858, t23861) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1870::<F>(t12461, t3698, t2019, t1983, t113, t1976, t22594, t22599, t22600, t22605, t22608, t22610, t22612, t22614, t22616, t22618, t22619, t22950, t2312, t2364, t23829, t23833, t23835, t23837, t23855, t510, t574, t6517, t652);
    (t23821, t23829, t23831, t23855, t23857, t23858, t23861)
}
