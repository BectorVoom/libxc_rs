//! MGGA_X_2D_PRHG07 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_2d_prhg07.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::bessel::{xc_bessel_I0, xc_bessel_I1};
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::lambert_w::{lambert_w};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_2d_prhg07_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t7 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t11 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t12 = -t8;
        let t13 = rho0 - rho1;
        let t15 = piecewise5::<f64>(t7, t8, t11, t12, t13 * t4);
        let t16 = 1.0 + t15;
        let t17 = t16 <= zeta_threshold;
        let t18 = f64::sqrt(zeta_threshold);
        let t19 = t18 * zeta_threshold;
        let t20 = f64::sqrt(t16);
        let t21 = t20 * t16;
        let t22 = piecewise3::<f64>(t17, t19, t21);
        let t23 = M_PI * t22;
        let t24 = M_SQRT2;
        let t25 = f64::sqrt(t3);
        let t26 = t24 * t25;
        let t27 = rho0 * rho0;
        let t28 = 1.0 / t27;
        let t32 = t27 * rho0;
        let t33 = 1.0 / t32;
        let t37 = 1.0 / M_PI;
        let t38 = (lapl0 * t28 / 4.0 - tau0 * t28 + sigma0 * t33 / 8.0) * t37;
        let t39 = -0.9999999999e0 < t38;
        let t40 = piecewise3::<f64>(t39, t38, -0.9999999999e0);
        let t41 = f64::exp(-1.0);
        let t43 = lambert_w::<f64>(t40 * t41);
        let t44 = t43 + 1.0;
        let t45 = t44 / 2.0;
        let t46 = xc_bessel_I0::<f64>(t45);
        let t47 = t26 * t46;
        let t50 = piecewise3::<f64>(t2, 0.0, -t23 * t47 / 8.0);
        let t51 = rho1 <= dens_threshold;
        let t52 = -t13;
        let t54 = piecewise5::<f64>(t11, t8, t7, t12, t52 * t4);
        let t55 = 1.0 + t54;
        let t56 = t55 <= zeta_threshold;
        let t57 = f64::sqrt(t55);
        let t58 = t57 * t55;
        let t59 = piecewise3::<f64>(t56, t19, t58);
        let t60 = M_PI * t59;
        let t61 = rho1 * rho1;
        let t62 = 1.0 / t61;
        let t66 = t61 * rho1;
        let t67 = 1.0 / t66;
        let t71 = (lapl1 * t62 / 4.0 - tau1 * t62 + sigma2 * t67 / 8.0) * t37;
        let t72 = -0.9999999999e0 < t71;
        let t73 = piecewise3::<f64>(t72, t71, -0.9999999999e0);
        let t75 = lambert_w::<f64>(t73 * t41);
        let t76 = t75 + 1.0;
        let t77 = t76 / 2.0;
        let t78 = xc_bessel_I0::<f64>(t77);
        let t79 = t26 * t78;
        let t82 = piecewise3::<f64>(t51, 0.0, -t60 * t79 / 8.0);
        let tzk0 = t50 + t82;
        zk[ip] += tzk0;
        let t83 = t3 * t3;
        let t84 = 1.0 / t83;
        let t85 = t13 * t84;
        let t87 = piecewise5::<f64>(t7, 0.0, t11, 0.0, t4 - t85);
        let t90 = piecewise3::<f64>(t17, 0.0, 3.0 / 2.0 * t20 * t87);
        let t91 = M_PI * t90;
        let t95 = t24 / t25;
        let t96 = t95 * t46;
        let t98 = t23 * t96 / 16.0;
        let t99 = t23 * t26;
        let t100 = xc_bessel_I1::<f64>(t45);
        let t105 = t27 * t27;
        let t106 = 1.0 / t105;
        let t111 = piecewise3::<f64>(t39, (-lapl0 * t33 / 2.0 + 2.0 * tau0 * t33 - 3.0 / 8.0 * sigma0 * t106) * t37, 0.0);
        let t113 = 1.0 / t44;
        let t114 = t43 * t113;
        let t116 = t114 / t40;
        let t117 = t100 * t111 * t116;
        let t121 = piecewise3::<f64>(t2, 0.0, -t91 * t47 / 8.0 - t98 - t99 * t117 / 16.0);
        let t122 = t52 * t84;
        let t124 = piecewise5::<f64>(t11, 0.0, t7, 0.0, -t4 - t122);
        let t127 = piecewise3::<f64>(t56, 0.0, 3.0 / 2.0 * t57 * t124);
        let t128 = M_PI * t127;
        let t131 = t95 * t78;
        let t133 = t60 * t131 / 16.0;
        let t135 = piecewise3::<f64>(t51, 0.0, -t128 * t79 / 8.0 - t133);
        let tvrho0 = t50 + t82 + t3 * (t121 + t135);
        vrho[ip * 2] += tvrho0;
        let t139 = piecewise5::<f64>(t7, 0.0, t11, 0.0, -t4 - t85);
        let t142 = piecewise3::<f64>(t17, 0.0, 3.0 / 2.0 * t20 * t139);
        let t143 = M_PI * t142;
        let t147 = piecewise3::<f64>(t2, 0.0, -t143 * t47 / 8.0 - t98);
        let t149 = piecewise5::<f64>(t11, 0.0, t7, 0.0, t4 - t122);
        let t152 = piecewise3::<f64>(t56, 0.0, 3.0 / 2.0 * t57 * t149);
        let t153 = M_PI * t152;
        let t156 = t60 * t26;
        let t157 = xc_bessel_I1::<f64>(t77);
        let t162 = t61 * t61;
        let t163 = 1.0 / t162;
        let t168 = piecewise3::<f64>(t72, (-lapl1 * t67 / 2.0 + 2.0 * tau1 * t67 - 3.0 / 8.0 * sigma2 * t163) * t37, 0.0);
        let t170 = 1.0 / t76;
        let t171 = t75 * t170;
        let t173 = t171 / t73;
        let t174 = t157 * t168 * t173;
        let t178 = piecewise3::<f64>(t51, 0.0, -t153 * t79 / 8.0 - t133 - t156 * t174 / 16.0);
        let tvrho1 = t50 + t82 + t3 * (t147 + t178);
        vrho[ip * 2 + 1] += tvrho1;
        let t181 = t33 * t37;
        let t183 = piecewise3::<f64>(t39, t181 / 8.0, 0.0);
        let t184 = t100 * t183;
        let t185 = t184 * t116;
        let t188 = piecewise3::<f64>(t2, 0.0, -t99 * t185 / 16.0);
        let tvsigma0 = t3 * t188;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t189 = t67 * t37;
        let t191 = piecewise3::<f64>(t72, t189 / 8.0, 0.0);
        let t192 = t157 * t191;
        let t193 = t192 * t173;
        let t196 = piecewise3::<f64>(t51, 0.0, -t156 * t193 / 16.0);
        let tvsigma2 = t3 * t196;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t197 = t28 * t37;
        let t199 = piecewise3::<f64>(t39, t197 / 4.0, 0.0);
        let t200 = t100 * t199;
        let t201 = t200 * t116;
        let t204 = piecewise3::<f64>(t2, 0.0, -t99 * t201 / 16.0);
        let tvlapl0 = t3 * t204;
        vlapl[ip * 2] += tvlapl0;
        let t205 = t62 * t37;
        let t207 = piecewise3::<f64>(t72, t205 / 4.0, 0.0);
        let t208 = t157 * t207;
        let t209 = t208 * t173;
        let t212 = piecewise3::<f64>(t51, 0.0, -t156 * t209 / 16.0);
        let tvlapl1 = t3 * t212;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t213 = piecewise3::<f64>(t39, -t197, 0.0);
        let t214 = t100 * t213;
        let t215 = t214 * t116;
        let t218 = piecewise3::<f64>(t2, 0.0, -t99 * t215 / 16.0);
        let tvtau0 = t3 * t218;
        vtau[ip * 2] += tvtau0;
        let t219 = piecewise3::<f64>(t72, -t205, 0.0);
        let t220 = t157 * t219;
        let t221 = t220 * t173;
        let t224 = piecewise3::<f64>(t51, 0.0, -t156 * t221 / 16.0);
        let tvtau1 = t3 * t224;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
