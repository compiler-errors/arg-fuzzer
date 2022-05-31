
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3805(_: S1, _: S2, _: S5, _: S6) {}
        
        fn test3805() { foo3805(S7, S4, S1, S1, S4, S7, S1, S2); }
    