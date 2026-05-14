//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1300/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1300<F: Float>(t43776: F, t43759: F, t43766: F, t43768: F, t43770: F, t43773: F, t43833: F, t43835: F, t43837: F, t43839: F, t43842: F, t43845: F, t43848: F, t43851: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t43866: F, t43869: F, t43872: F, t43875: F, t43882: F, t43884: F, t43887: F, t43890: F, t43892: F) -> (F, F) {
    let t44027 = 0.13388493827160493828e1 * t43776;
    let t44036 = 0.49293999999999999999e0 * t43759 - 0.85199506172839506175e-1 * t43766 + 0.21908444444444444444e0 * t43768 - 0.13145066666666666666e1 * t43770 + 0.21908444444444444444e0 * t43773 + t44027 + 0.3071625e0 * t43833 + 0.43816888888888888888e0 * t43835 - 0.13145066666666666666e1 * t43837 - 0.21908444444444444444e0 * t43839 + 0.43816888888888888889e0 * t43842 - 0.98587999999999999998e0 * t43845 + 0.197176e1 * t43848 + 0.82156666666666666667e-1 * t43851;
    let t44052 = -0.18257037037037037037e0 * t43855 - 0.97370864197530864196e-1 * t43857 - 0.97370864197530864199e0 * t43859 + 0.54771111111111111111e0 * t43861 + 0.10954222222222222222e1 * t43863 - 0.379785e1 * t43866 + 0.614325e0 * t43869 + 0.85451625e1 * t43872 - 0.46074375e0 * t43875 - 0.3560484375e1 * t43882 + 0.1898925e1 * t43884 - 0.28483875e1 * t43887 + 0.1151859375e0 * t43890 + 0.46074375e0 * t43892;
    (t44036, t44052)
}
