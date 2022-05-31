
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4537(_: S1, _: S3, _: S5, _: S6, _: S7) {}
        
        fn test4537() { foo4537(S1, S3, S4, S5, S7, S8); }
    