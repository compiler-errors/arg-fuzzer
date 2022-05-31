
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8298(_: S7, _: S6, _: S2, _: S1, _: S8) {}
        
        fn test8298() { foo8298(S6, S4, S4, S5, S3, S2, S3); }
    