
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16529(_: S2, _: S4, _: S5) {}
        
        fn test16529() { foo16529(S2, S3, S4, S8); }
    