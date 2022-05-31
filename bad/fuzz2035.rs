
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2035(_: S8, _: S5, _: S7, _: S6, _: S7) {}
        
        fn test2035() { foo2035(S6, S8, S6, S5, S6, S3, S1, S7); }
    