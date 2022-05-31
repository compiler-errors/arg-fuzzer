
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo83(_: S5, _: S8, _: S3, _: S4, _: S7, _: S1, _: S6, _: S2) {}
        
        fn test83() { foo83(S2, S7, S5); }
    