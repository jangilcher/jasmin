(* ** License
 * -----------------------------------------------------------------------
 * Copyright 2016--2017 IMDEA Software Institute
 * Copyright 2016--2017 Inria
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 * ----------------------------------------------------------------------- *)

(* ** Imports and settings *)
From mathcomp Require Import all_ssreflect all_algebra.
From CoqWord Require Import ssrZ.
Require Import xseq.
Require Export xseq ZArith strings word utils ident type.

Set Implicit Arguments.
Unset Strict Implicit.
Unset Printing Implicit Defensive.

Local Unset Elimination Schemes.

(* ---------------------------------------------------------------------- *)

Record global := Global { size_of_global : wsize ; ident_of_global:> Ident.ident }.

Definition global_beq (g1 g2: global) : bool :=
  let 'Global s1 n1 := g1 in
  let 'Global s2 n2 := g2 in
  (s1 == s2) && (n1 == n2).

Lemma global_eq_axiom : Equality.axiom global_beq.
Proof.
  case => s1 g1 [] s2 g2 /=; case: andP => h; constructor.
  - by case: h => /eqP -> /eqP ->.
  by case => ??; apply: h; subst.
Qed.

Definition global_eqMixin := Equality.Mixin global_eq_axiom.
Canonical global_eqType := Eval hnf in EqType global global_eqMixin.

(* ---------------------------------------------------------------------- *)

Definition glob_decl := (global * Z)%type.
Notation glob_decls  := (seq glob_decl).

(* ---------------------------------------------------------------------- *)
Definition get_global_Z (gd: glob_decls) (g: global) : option Z :=
  assoc gd g.

Definition get_global_word gd g : exec (word (size_of_global g)) :=
  if get_global_Z gd g is Some z then
    ok (wrepr (size_of_global g) z)
  else type_error.

