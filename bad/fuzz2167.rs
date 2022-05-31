
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2167(_: S3, _: S4, _: S8) {}
        
        fn test2167() { foo2167(S1, S3, S4, S5, S6); }
    