//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta306 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1093;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1094;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1095;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1096;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1097;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1098;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1099;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1100;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta306<F: Float>(t601: F, t9238: F, t85: F, t24: F, t2241: F, t2307: F, t10276: F, t73: F, t2244: F, t2250: F, t16: F, t39031: F, t25: F, t28: F, zeta_threshold: F, t11152: F, t76: F, t2251: F, t2252: F, t2255: F, t2283: F, t2284: F, t2291: F, t2298: F, t2304: F, t608: F, t609: F, t629: F, t634: F, t638: F, t642: F, t66: F, t72: F, t80: F, t9258: F, t9263: F, t9268: F, t9312: F, t9313: F, t9321: F, t9324: F, t9330: F, t9333: F, t9339: F, t41: F, t42: F, t53: F, t54: F, t1028: F, t36: F, t9576: F, t2262: F, t2267: F, t2268: F, t2271: F, t2274: F, t39: F, t43: F, t44: F, t51: F, t55: F, t615: F, t618: F, t9277: F, t9287: F, t9289: F, t9292: F, t9293: F, t9296: F, t9300: F, t9304: F, sigma0: F, t1864: F, t2245: F, t31: F, t33: F, t607: F, t628: F, t65: F, t6509: F, t67: F, t9247: F, t9248: F, t9251: F, t9259: F, t9260: F, t2235: F, t2240: F, t39030: F, t39032: F, t39034: F, t39036: F, t39038: F, t39040: F, t39043: F, t39046: F, t39049: F, t605: F, t645: F, t86: F, t9228: F, t9231: F, t9239: F, t9240: F, t9243: F, t9342: F, t5: F, t112: F, t2363: F, t111: F, t9346: F, t2405: F, t2420: F, t702: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t39054, t39063, t39064, t39070, t39096, t39097) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1093::<F>(t601, t9238, t85, t24, t2241, t2307, t10276, t73, t2244);
        let t39103 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1094::<F>(t2250);
        let (t39108, t39109) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1095::<F>(t16, t39031);
        let t39110 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1096::<F>(t25, t28, t39109, zeta_threshold);
        let t39130 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1097::<F>(t11152, t76, t2244, t2250, t2251, t2252, t2255, t2283, t2284, t2291, t2298, t2304, t39096, t39097, t39103, t39110, t608, t609, t629, t634, t638, t642, t66, t72, t80, t9258, t9263, t9268, t9312, t9313, t9321, t9324, t9330, t9333, t9339);
        let (t39177, t39213) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1098::<F>(t41, t42, t53, t54, t1028, t36, t9576, t2244, t2250, t2262, t2267, t2268, t2271, t2274, t39, t39097, t39103, t39110, t43, t44, t51, t55, t615, t618, t9258, t9277, t9287, t9289, t9292, t9293, t9296, t9300, t9304, sigma0);
        let t39217 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1099::<F>(t1864, t2244, t2245, t2250, t2283, t2304, t31, t33, t39103, t39110, t39213, t607, t628, t642, t65, t6509, t67, t80, t9247, t9248, t9251, t9258, t9259, t9260);
        let t39221 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1100::<F>(t2235, t2240, t2241, t2307, t39030, t39032, t39034, t39036, t39038, t39040, t39043, t39046, t39049, t39054, t39063, t39064, t39070, t39130, t39217, t605, t645, t86, t9228, t9231, t9239, t9240, t9243, t9342);
        let (t39223, t39231, t39235, t39246, t39249) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1101::<F>(t5, t39221, t112, t2363, t111, t9346, t2405, t2420, t702);
    (t39097, t39103, t39108, t39109, t39110, t39177, t39223, t39231, t39235, t39246, t39249)
}
