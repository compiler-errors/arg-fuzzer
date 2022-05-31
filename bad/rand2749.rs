
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2749(_: S4, _: S5, _: S6, _: S7) {}
        
        fn test2749() { foo2749(S2, S2, S5, S7, S3); }
    