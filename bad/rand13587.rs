
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13587(_: S3, _: S6) {}
        
        fn test13587() { foo13587(S4, S7, S3, S5, S2); }
    