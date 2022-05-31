
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6846(_: S1, _: S2, _: S6, _: S7) {}
        
        fn test6846() { foo6846(S1, S2, S3, S2, S1, S6, S3); }
    