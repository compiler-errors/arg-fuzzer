
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9645(_: S2, _: S4, _: S7) {}
        
        fn test9645() { foo9645(S5, S1, S2, S3, S2); }
    