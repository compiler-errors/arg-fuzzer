
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2393(_: S6, _: S5, _: S3, _: S4, _: S1) {}
        
        fn test2393() { foo2393(S1, S2, S3, S5, S6, S7, S8); }
    