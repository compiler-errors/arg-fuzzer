
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10449(_: S2, _: S5, _: S8) {}
        
        fn test10449() { foo10449(S1, S2, S3, S5, S7); }
    