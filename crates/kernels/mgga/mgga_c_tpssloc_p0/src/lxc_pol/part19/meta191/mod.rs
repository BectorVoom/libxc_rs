//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta191 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk850;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk851;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta191<F: Float>(t10103: F, t858: F, t856: F, t68: F, t2719: F, t865: F, t2742: F, t2718: F, t10047: F, t10049: F, t259: F, t2597: F, t2713: F, t2720: F, t2743: F, t855: F, t866: F, t9520: F, t9585: F, t9587: F, t9590: F, t9593: F, t193: F, t202: F, t2379: F, t2522: F, t2523: F, t2553: F, t262: F, t4314: F, t766: F, t776: F, t870: F, t9450: F, t9457: F, t9458: F, t9463: F, t9469: F, t9470: F, t9476: F, t9484: F, t9496: F, t9516: F, t2745: F, t2752: F, t1877: F, t868: F, t9684: F, t9715: F, t9718: F, t9724: F, t9727: F, t9780: F, t9789: F, t9863: F, t9865: F, t9867: F, t9870: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10104, t10108, t10110, t10112, t10116, t10121) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk850::<F>(t10103, t858, t856, t68, t2719, t865, t2742, t2718, t10047, t10049, t259, t2597, t2713, t2720, t2743, t855, t866, t9520, t9585, t9587, t9590, t9593);
        let t10125 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk851::<F>(t10121, t193, t202, t2379, t2522, t2523, t2553, t262, t4314, t766, t776, t870, t9450, t9457, t9458, t9463, t9469, t9470, t9476, t9484, t9496, t9516);
        let (t10126, t10134, t10138) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk852::<F>(t2745, t870, t2553, t262, t2752, t1877, t2522, t4314, t776, t868, t9684, t9715, t9718, t9724, t9727, t9780, t9789, t9863, t9865, t9867, t9870);
    (t10104, t10108, t10110, t10112, t10116, t10121, t10125, t10126, t10134, t10138)
}
